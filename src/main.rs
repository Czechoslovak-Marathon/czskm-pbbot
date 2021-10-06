use serenity::{
    async_trait,
    model::{channel::Message, id::ChannelId, gateway::Ready},
    utils::Colour,
    prelude::*,
};
use anyhow::Result;
use tokio::time::{sleep, Duration};

use crate::{apirequests::*};
use crate::{apitypes::*};
use crate::{database::*};

pub mod apitypes;
pub mod apirequests;
pub mod database;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Add new runner with !srcadd command
        if msg.content.starts_with("!srcadd ") && msg.channel_id == 788595458729574400 {
            match &msg.member {
                Some(member) => {
                    let found_role : bool = member.roles.iter().any(
                        |&x| x == 467012114725470240
                    );
                    if found_role {
                        let mut data = ctx.data.write().await;
                        let db = data.get_mut::<Database>().unwrap();
                        let runner : Vec<_> = msg.content.split("!srcadd ").collect();
                        let latest_run = get_latest_run(runner[1]).await;
                        let run_id: String = match latest_run {
                            Ok(latest_run) => {
                                match latest_run {
                                    Some(x) => {
                                        x.run.id
                                    }
                                    None => {
                                        println!("[INFO] Runner has no runs");
                                        String::from("")
                                    }
                                }
                            }
                            Err(_) => {
                                println!("[ERROR] Failed to get latest run");
                                return
                            }
                        };
                        let added = db.add_runner(runner[1], &run_id).await;
                        match added {
                            Ok(_) => println!("[INFO] Added new runner"),
                            Err(_) => println!("[ERROR] Failed to add runner")
                        }
                    }
                }
                None => println!("[WARN] User is not in the Guild")
            }
            match msg.delete(&ctx.http).await {
                Ok(_) => return,
                Err(_) => println!("[ERROR] Failed to delete message")
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("[INFO] {} is connected!", ready.user.name);
        loop {
            // Infinitely loop through all runners in database
            let runners: Result<Vec<Runner>>;
            {
                let mut data = ctx.data.write().await;
                let db = data.get_mut::<Database>().unwrap();
                runners = db.get_runners().await;
            }
            let runners_vector: Vec<Runner> = match runners {
                Ok(r) => r,
                Err(_) => {
                    println!("[ERROR] Couldn't get runners");
                    return;
                }
            };
            println!("[INFO] Got runners, iterating");
            for runner in runners_vector {
                loop {
                    // Sleep to prevent spamming the API
                    sleep(Duration::from_millis(1000)).await;
                    // Get latest run from the API
                    match get_latest_run(&runner.name).await {
                        Ok(latest_run) => {
                            match latest_run {
                                Some(run) => {
                                    // Run was found, check if it's new, then get other info
                                    if run.run.id != runner.last_run {
                                        // Get game info from the API
                                        let game: Option<Game> = match get_game_data(&run.run.game).await {
                                            Ok(game_data) => game_data,
                                            Err(_) => {
                                                println!("[ERROR] Failed to get game info");
                                                continue;
                                            }
                                        };

                                        // Get category info from the API
                                        let category: Option<Category> = match get_category_data(&run.run.category).await {
                                            Ok(category_data) => category_data,
                                            Err(_) => {
                                                println!("[ERROR] Failed to get category info");
                                                continue;
                                            }
                                        };

                                        // Get level info from the API
                                        let level: Option<Level> = match &run.run.level {
                                            None => {
                                                println!("[INFO] Run has no level");
                                                None
                                            }
                                            Some(level) => {
                                                match get_level_data(&level).await {
                                                    Ok(level_data) => level_data,
                                                    Err(_) => {
                                                        println!("[ERROR] Failed to get level info");
                                                        continue;
                                                    }
                                                }
                                            }
                                        };
                                        
                                        // Get variables from the API
                                        let variables: Option<String> = match get_variables(run.run.values.clone()).await {
                                            Ok(variables_data) => variables_data,
                                            Err(_) => {
                                                println!("[ERROR] Failed to get variables");
                                                continue;
                                            }
                                        };

                                        // Preparing data for Embed
                                        let game: Game = match game {
                                            Some(game) => game,
                                            None => continue
                                        };
                                        let category: String = match category {
                                            Some(category) => category.name,
                                            None => continue
                                        };
                                        let level: String = match level {
                                            Some(level) => level.name,
                                            None => String::from("")
                                        };
                                        let variables: String = match variables {
                                            Some(variables) => format!(" ({})", variables),
                                            None => String::from("")
                                        };

                                        // Creating Embed
                                        if let Err(why) = ChannelId(788595458729574400)
                                            .send_message(&ctx, |m| {
                                                m.embed(|e| {
                                                    if level == "" {
                                                        e.title(format!("{} — {}{}", game.names.international, category, variables));
                                                    }
                                                    else {
                                                        e.title(format!("{} — {} {}{}", game.names.international, level, category, variables));
                                                    }
                                                    let time: String = format_time(run.run.times.primary_t);
                                                    e.description(format!("**[{} by {}]({})**", time, &runner.name, &run.run.weblink));
                                                    let colour: Colour = match &run.place {
                                                        1 => Colour::GOLD,
                                                        2 => Colour::LIGHT_GREY,
                                                        3 => Colour::DARK_ORANGE,
                                                        _ => Colour::RED
                                                    };
                                                    e.colour(colour);
                                                    e.thumbnail(game.assets.cover_medium.uri);
                                                    e.field("Leaderboard rank:", &run.place, false);
                                                    e.field("Date played:", &run.run.date, false);
                                                    e
                                                })
                                            })
                                            .await
                                        {
                                            println!("[ERROR] Failed to send message: {:?}", why);
                                        };

                                        // Updating runner's last run in the database
                                        {
                                            let mut data = ctx.data.write().await;
                                            let db = data.get_mut::<Database>().unwrap();
                                            match db.update_runner(runner.name, run.run.id).await {
                                                Ok(_) => println!("[INFO] Updated runner"),
                                                Err(_) => print!("[ERROR] Failed to update runner")
                                            };
                                        }
                                    }
                                    break;
                                }
                                None => println!("[INFO] Runner has no runs")
                            }
                        },
                        Err(e) => {
                            println!("[ERROR] Failed to get latest run for {}", runner.name);
                            println!("{:?}", e);
                            continue;
                        }
                    }
                    break;
                }
            }
        }
    }
}

fn format_time(time: f64) -> String {
    let duration: Duration = Duration::from_millis((time * 1000.0) as u64);
    let seconds = (duration.as_millis() as f64) / 1000.0 % 60.0;
    let minutes = ((duration.as_millis() / 1000 / 60) % 60) as u64;
    let hours = ((duration.as_millis() / 1000 / 60) / 60) as u64;
    let is_decimal = !(seconds.fract() == 0.0);
    let time_string: String;
    if hours != 0 {
        if is_decimal {
            time_string = format!("{}h {:2}m {:2.3}s", hours, minutes, seconds);
        } else {
            time_string = format!("{}h {:2}m {:2}s", hours, minutes, seconds);
        }
    }
    else if minutes != 0 {
        if is_decimal {
            time_string = format!("{}m {:2.3}s", minutes, seconds);
        } else {
            time_string = format!("{}m {:2}s", minutes, seconds);
        }
    }
    else {
        if is_decimal {
            time_string = format!("{:2.3}s", seconds);
        } else {
            time_string = format!("{}s", seconds as u64);
        }
    }
    time_string
}

#[tokio::main]
async fn main() {
    // Discord Token (better as environmental variable)
    let token = "";
    let mut client = Client::builder(&token).event_handler(Handler).await.expect("[ERROR] Error creating client");
    {
        // Add Database into client's data
        let mut w = client.data.write().await;
        match connect() {
            Ok(db) => w.insert::<Database>(db),
            Err(_)  => println!("[ERROR] Database failed to connect")
        }
    }
    // Start Discord bot
    if let Err(why) = client.start().await {
        println!("[ERROR] Client error: {:?}", why)
    }
}
