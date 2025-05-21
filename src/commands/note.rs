use serenity::model::application::ResolvedOption;
use serenity::builder::CreateCommand;
use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommandOption, CreateEmbed, CreateInteractionResponseFollowup, InstallationContext, ResolvedValue, UserId};
use sqlx::{Pool, Postgres};

struct Note {
    id: i64,
    name: String,
    note: String,
    owner: i64,
    context: i64,
}

pub fn register() -> CreateCommand {
    CreateCommand::new("note")
        .description("Store and Retrieve Notes")
        .add_integration_type(InstallationContext::Guild)
        .add_integration_type(InstallationContext::User)
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "create", "Creates a new note")
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "name", "The name of the note to create").required(true)
                )
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "note", "The note to be saved").required(true)
                )
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "context", "Whether to find your user or server notes")
                        .add_string_choice("Server", "server")
                        .add_string_choice("User", "user")
                )
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "get", "Gets a note from the database")
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "name", "The name of the note to get").required(true)
                )
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "context", "Whether to find your user or server notes")
                        .add_string_choice("Server", "server")
                        .add_string_choice("User", "user")
                )
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::Boolean,"hidenote", "Whether the note should only show for you or for everyone")
                )
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "list", "Lists all of the tags you own")
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "context", "Whether to find your user or server notes")
                        .add_string_choice("Server", "server")
                        .add_string_choice("User", "user")
                )
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "delete", "Removes a note if you own it")
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "name", "The name of the note to delete").required(true)
                )
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "context", "Whether to find your user or server notes")
                        .add_string_choice("Server", "server")
                        .add_string_choice("User", "user")
                )
        )
}

pub async fn execute<'a>(options: &[ResolvedOption<'a>], interaction: CommandInteraction, ctx: Context, db: Pool<Postgres>) -> CreateInteractionResponseFollowup {
    for option in options {
        match &option.value {
            ResolvedValue::SubCommand(sub_options) => {
                match &option.name[..] { // Sub command
                    "create" => {
                        let mut name = None;
                        let mut note = None;
                        let mut context = None;

                        for option in sub_options { // Get values from interaction
                            match (&option.name[..], &option.value) {
                                ("name", ResolvedValue::String(s)) => name = Some(s),
                                ("note", ResolvedValue::String(s)) => note = Some(s),
                                ("context", ResolvedValue::String(s)) => context = Some(s),
                                _ => {}
                            }
                        };

                        let query_context = match context {
                            Some(&"server") => {
                                let guild_id = interaction.guild_id;

                                match guild_id {
                                    Some(gid) => {
                                        gid.get()
                                    }
                                    None => {
                                        interaction.user.id.get()
                                    }
                                }
                            }
                            Some(&"user") => {
                                interaction.user.id.get()
                            }
                            Some(&_a) => {
                                let guild_id = interaction.guild_id;

                                match guild_id {
                                    Some(gid) => {
                                        gid.get()
                                    }
                                    None => {
                                        interaction.user.id.get()
                                    }
                                }
                            }
                            None => {
                                let guild_id = interaction.guild_id;

                                match guild_id {
                                    Some(gid) => {
                                        gid.get()
                                    }
                                    None => {
                                        interaction.user.id.get()
                                    }
                                }
                            }
                        };

                        let exists = sqlx::query_as!( // Check if the note already exists
                            Note,
                            "SELECT * FROM bot_hd_notes WHERE name = $1 AND context = $2",
                            name,
                            query_context as i64,
                        )
                        .fetch_optional(&db)
                        .await;

                        match exists {
                            Ok(Some(a)) => { // Note already exists - error
                                let owner_id = UserId::new(a.owner as u64);
                                let owner = owner_id.to_user(&ctx.http).await.unwrap();
                                let owner_name = owner.display_name();

                                let context_text = if a.context as u64 == interaction.user.id.get() {
                                    String::from("user")
                                } else {
                                    String::from("server")
                                };

                                let response_message = format!("ERROR: Note already exists\n`{}`'s {} note `{}`:\n{}", *name.unwrap(), owner_name, context_text, a.note);
                                return CreateInteractionResponseFollowup::new().content(response_message).ephemeral(true);
                            }
                            Ok(None) => { // Note doesn't exist - make it!
                                let response = sqlx::query!(
                                    "INSERT INTO bot_hd_notes (name, note, owner, context) VALUES ($1, $2, $3, $4)",
                                    name,
                                    note,
                                    interaction.user.id.get() as i64,
                                    query_context as i64,
                                )
                                .execute(&db)
                                .await;

                                match response {
                                    Ok(_) => {
                                        let response_message = format!("Successfully created note `{}`", *name.unwrap());
                                        return CreateInteractionResponseFollowup::new().content(response_message).ephemeral(true);
                                    }
                                    Err(_) => {
                                        return CreateInteractionResponseFollowup::new().content("ERROR: Failed to query database when saving note").ephemeral(true);
                                    }
                                }
                            }
                            Err(_) => {
                                return CreateInteractionResponseFollowup::new().content("ERROR: Failed to query database").ephemeral(true);
                            }
                        }
                    }
                    "get" => {
                        let mut name = None;
                        let mut hidenote = None;
                        let mut context = None;

                        for option in sub_options { // Get values from interaction
                            match (&option.name[..], &option.value) {
                                ("name", ResolvedValue::String(s)) => name = Some(s),
                                ("hidenote", ResolvedValue::Boolean(b)) => hidenote = Some(b),
                                ("context", ResolvedValue::String(s)) => context = Some(s),
                                _ => {}
                            }
                        };

                        let query_context = match context {
                            Some(&"server") => {
                                let guild_id = interaction.guild_id;

                                match guild_id {
                                    Some(gid) => {
                                        gid.get()
                                    }
                                    None => {
                                        interaction.user.id.get()
                                    }
                                }
                            }
                            Some(&"user") => {
                                interaction.user.id.get()
                            }
                            Some(&_a) => {
                                let guild_id = interaction.guild_id;

                                match guild_id {
                                    Some(gid) => {
                                        gid.get()
                                    }
                                    None => {
                                        interaction.user.id.get()
                                    }
                                }
                            }
                            None => {
                                let guild_id = interaction.guild_id;

                                match guild_id {
                                    Some(gid) => {
                                        gid.get()
                                    }
                                    None => {
                                        interaction.user.id.get()
                                    }
                                }
                            }
                        };

                        let response = sqlx::query_as!(
                            Note,
                            "SELECT * FROM bot_hd_notes WHERE name = $1 AND context = $2",
                            name,
                            query_context as i64,
                        )
                        .fetch_optional(&db)
                        .await;

                        match response {
                            Ok(Some(a)) => { // Note exists - Display to the user
                                let owner_id = UserId::new(a.owner as u64);
                                let owner = owner_id.to_user(&ctx.http).await.unwrap();
                                let owner_name = owner.display_name();

                                let context_text = if a.context as u64 == interaction.user.id.get() {
                                    String::from("user")
                                } else {
                                    String::from("server")
                                };

                                let response_text = format!(
                                    "`{}`'s {} note `{}`:\n{}",
                                    owner_name,
                                    context_text,
                                    a.name,
                                    a.note
                                );
                                
                                match hidenote {
                                    Some(b) => {
                                        return CreateInteractionResponseFollowup::new().content(response_text).ephemeral(*b);
                                    }
                                    None => {
                                        return CreateInteractionResponseFollowup::new().content(response_text)
                                    }
                                }
                            }
                            Ok(None) => { // Note doesn't exist
                                let response_text = format!("The note `{}` doesn't exist, please specify a note that exists", *name.unwrap());
                                return CreateInteractionResponseFollowup::new().content(response_text).ephemeral(true);
                            }
                            Err(_) => {
                                return CreateInteractionResponseFollowup::new().content("ERROR: Failed to query database").ephemeral(true);
                            }
                        }
                    }
                    "list" => {
                        let mut context = None;

                        for option in sub_options { // Get values from interaction
                            match (&option.name[..], &option.value) {
                                ("context", ResolvedValue::String(s)) => context = Some(s),
                                _ => {}
                            }
                        };

                        let query_context = match context {
                            Some(&"server") => {
                                let guild_id = interaction.guild_id;

                                match guild_id {
                                    Some(gid) => {
                                        gid.get()
                                    }
                                    None => {
                                        interaction.user.id.get()
                                    }
                                }
                            }
                            Some(&"user") => {
                                interaction.user.id.get()
                            }
                            Some(&_a) => {
                                let guild_id = interaction.guild_id;

                                match guild_id {
                                    Some(gid) => {
                                        gid.get()
                                    }
                                    None => {
                                        interaction.user.id.get()
                                    }
                                }
                            }
                            None => {
                                let guild_id = interaction.guild_id;

                                match guild_id {
                                    Some(gid) => {
                                        gid.get()
                                    }
                                    None => {
                                        interaction.user.id.get()
                                    }
                                }
                            }
                        };

                        let notes = sqlx::query_as!(
                            Note,
                            "SELECT * FROM bot_hd_notes WHERE context = $1 AND owner = $2",
                            query_context as i64,
                            interaction.user.id.get() as i64
                        )
                        .fetch_all(&db)
                        .await;

                        match notes {
                            Ok(notes) => {
                                let context_text = match context {
                                    Some(s) => {
                                        *s
                                    }
                                    None => {
                                        "server"
                                    }
                                };

                                let embed_title = format!("{}'s {} notes", interaction.user.display_name(), context_text);

                                let note_text = {
                                    let mut v_text = String::new();

                                    for note in notes {
                                        v_text += &format!("{}\n", note.name);
                                    }

                                    v_text
                                };

                                let embed = CreateEmbed::new()
                                    .title(embed_title)
                                    .color(0x00afff)
                                    .field("Notes", note_text, true);

                                return CreateInteractionResponseFollowup::new().add_embed(embed).ephemeral(true);
                            }
                            Err(_) => {
                                return CreateInteractionResponseFollowup::new().content("Failed to query database - either you own no notes or the database failed").ephemeral(true);
                            }
                        }
                    }
                    "delete" => {
                        let mut name = None;
                        let mut context = None;

                        for option in sub_options { // Get values from interaction
                            match (&option.name[..], &option.value) {
                                ("name", ResolvedValue::String(s)) => name = Some(s),
                                ("context", ResolvedValue::String(s)) => context = Some(s),
                                _ => {}
                            }
                        };
                        
                        let query_context = match context {
                            Some(&"server") => {
                                let guild_id = interaction.guild_id;

                                match guild_id {
                                    Some(gid) => {
                                        gid.get()
                                    }
                                    None => {
                                        interaction.user.id.get()
                                    }
                                }
                            }
                            Some(&"user") => {
                                interaction.user.id.get()
                            }
                            Some(&_a) => {
                                let guild_id = interaction.guild_id;

                                match guild_id {
                                    Some(gid) => {
                                        gid.get()
                                    }
                                    None => {
                                        interaction.user.id.get()
                                    }
                                }
                            }
                            None => {
                                let guild_id = interaction.guild_id;

                                match guild_id {
                                    Some(gid) => {
                                        gid.get()
                                    }
                                    None => {
                                        interaction.user.id.get()
                                    }
                                }
                            }
                        };

                        let exists = sqlx::query_as!(
                            Note,
                            "SELECT * FROM bot_hd_notes WHERE name = $1 AND context = $2",
                            name,
                            query_context as i64,
                        )
                        .fetch_optional(&db)
                        .await;

                        match exists {
                            Ok(Some(a)) => { // Note exists - Verify if the user owns it
                                if interaction.user.id.get() == a.owner as u64 {
                                    let response = sqlx::query!(
                                        "DELETE FROM bot_hd_notes WHERE name = $1 AND context = $2",
                                        name,
                                        query_context as i64
                                    )
                                    .execute(&db)
                                    .await;

                                    match response {
                                        Ok(_) => { // Note successfully deleted
                                            let response_text = format!("Successfully deleted note `{}`", String::from(*name.unwrap()));
                                            return CreateInteractionResponseFollowup::new().content(response_text).ephemeral(true);
                                        }
                                        Err(_) => { // Note failed to delete - query failed
                                            return CreateInteractionResponseFollowup::new().content("ERROR: Failed to query database").ephemeral(true);
                                        }
                                    }
                                } else {
                                    return CreateInteractionResponseFollowup::new().content("You do not have permission to delete this note as you don't own it").ephemeral(true);
                                }
                            }
                            Ok(None) => { // Note doesn't exist
                                let response_text = format!("The note `{}` doesn't exist, please specify a note that exists", *name.unwrap());
                                return CreateInteractionResponseFollowup::new().content(response_text).ephemeral(true);
                            }
                            Err(_) => {
                                return CreateInteractionResponseFollowup::new().content("ERROR: Failed to query database when verifying ownership").ephemeral(true);
                            }
                        }
                    }
                    _ => {return CreateInteractionResponseFollowup::new().content("ERROR: This subcommand is not implemented").ephemeral(true)}
                }
            }
            _ => {
                return CreateInteractionResponseFollowup::new().content("How the fuck did you even get this error discord requires that you select a subcommand when running commands with subcommands")
            }
        }
    };

    CreateInteractionResponseFollowup::new().content("ERROR: No Options Found").ephemeral(true)
}