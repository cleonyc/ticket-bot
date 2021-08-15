use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{
        channel::{Message, PermissionOverwrite, PermissionOverwriteType},
        Permissions,
    },
};

use crate::utils::utils::convert_to_id;
use crate::DATABASE;
#[command]
async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    {
        let db = DATABASE.get().unwrap().write().await;
        if !db
            .channel_is_ticket(msg.channel_id.0 as usize)
            .await
            .unwrap()
        {
            return Ok(());
        }
    }

    let unparsed_user = match args.single::<String>() {
        Ok(i) => i,
        Err(_) => {
            msg.reply_ping(
                &ctx.http,
                "Please suppy a valid user id or mention! (ex. v!blacklist 797192291324133426 or <@797192291324133426>)",
            )
            .await
            .unwrap();
            return Ok(());
        }
    };
    let user_id = match convert_to_id(unparsed_user) {
        Ok(i) => i,
        Err(_) => {
            msg.reply_ping(
                &ctx.http,
                "Please suppy a valid user id or mention! (ex. v!blacklist 797192291324133426 or <@797192291324133426>)",
            )
            .await
            .unwrap();
            return Ok(());
        }
    };
    let user = match ctx.http.get_user(user_id).await {
        Ok(i) => i,
        Err(_) => {
            msg.reply_ping(
                &ctx.http,
                "Please suppy a valid user id or mention! (ex. v!blacklist 797192291324133426 or <@797192291324133426>) ",
            )
            .await
            .unwrap();
            return Ok(());
        }
    };
    let overwrite = PermissionOverwrite {
        allow: Permissions::READ_MESSAGES,
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Member(user.id),
    };
    msg.channel_id
        .create_permission(&ctx.http, &overwrite)
        .await?;
    let overwrite = PermissionOverwrite {
        allow: Permissions::READ_MESSAGES,
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Member(user.id),
    };
    msg.channel_id
        .create_permission(&ctx.http, &overwrite)
        .await?;
    Ok(())
}
