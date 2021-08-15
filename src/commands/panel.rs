use crate::{DATABASE, SETTINGS};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{
        channel::{Message, ReactionType},
        id::ChannelId,
        interactions::ButtonStyle,
    },
};
use uuid::Uuid;

/// .panel <Category ID> "<Embed title>" "<Embed description>" "<Interaction Message>"
#[command]
async fn panel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut found = false;
    {
        let settings_lock = SETTINGS.write().await;
        let staff_roles: Vec<u64> = settings_lock
            .get("staff_roles")
            .expect("staff_roles not found!");
        for i in staff_roles {
            if msg
                .author
                .has_role(&ctx.http, msg.guild(&ctx.cache).await.unwrap().id, i)
                .await?
            {
                found = true;
                break;
            }
        }
    }
    if !found {
        return Ok(());
    }
    let cat_id = match args.single::<usize>() {
        Ok(cat) => cat,
        Err(_) => {
            msg.reply_ping(&ctx.http, 
                
r#"Invalid usage! 
Correct usage: 
```
panel <Category ID to open tickets in> <Channel ID to send message in> "<Embed title>" "<Embed description>" "<Interaction Message>"
```
"#).await?;

            return Ok(());
        }
    };
    let channel_id = match args.single::<usize>() {
        Ok(cat) => cat,
        Err(_) => {
            msg.reply_ping(&ctx.http, 
                
r#"Invalid usage! 
Correct usage: 
```
panel <Category ID to open tickets in> <Channel ID to send message in> "<Embed title>" "<Embed description>" "<Interaction Message>"
```
"#).await?;

            return Ok(());
        }
    };
    let title = match args.single_quoted::<String>() {
        Ok(cat) => cat,
        Err(_) => {
            msg.reply_ping(&ctx.http, 
                
r#"Invalid usage! 
Correct usage: 
```
panel <Category ID to open tickets in> <Channel ID to send message in> "<Embed title>" "<Embed description>" "<Interaction Message>"
```
"#).await?;

            return Ok(());
        }
    };
    let desc = match args.single_quoted::<String>() {
        Ok(cat) => cat,
        Err(_) => {
            msg.reply_ping(&ctx.http, 
                
r#"Invalid usage! 
Correct usage: 
```
panel <Category ID to open tickets in> <Channel ID to send message in> "<Embed title>" "<Embed description>" "<Interaction Message>"
```
"#).await?;

            return Ok(());
        }
    };
    let inter_message = match args.single_quoted::<String>() {
        Ok(cat) => cat,
        Err(_) => {
            msg.reply_ping(&ctx.http, 
                
r#"Invalid usage! 
Correct usage: 
```
panel <Category ID to open tickets in> <Channel ID to send message in> "<Embed title>" "<Embed description>" "<Interaction Message>"
```
"#).await?;

            return Ok(());
        }
    };
    let uuid = Uuid::new_v4();
    let msg = ChannelId(channel_id as u64)
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(title);
                e.description(desc);
                e
            });
            m.components(|c| {
                c.create_action_row(|a| {
                    a.create_button(|b| {
                        b.custom_id(uuid.to_hyphenated().to_string());
                        b.label(inter_message);
                        b.emoji(ReactionType::Unicode("📩".to_string()));
                        b.style(ButtonStyle::Primary)
                    });
                    a
                })
            });
            m
        })
        .await?;

    {
        let db = DATABASE.get().unwrap().write().await;
        db.new_panel(msg.id.0 as usize, cat_id, uuid).await?
    }

    Ok(())
}
