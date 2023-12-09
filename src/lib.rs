use discord_rich_presence::{
    activity::{self, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use reaper_high::Reaper;
use reaper_macros::reaper_extension_plugin;
use reaper_medium::{ControlSurface, PlayState};
use std::{
    error::Error,
    fmt::{self, Debug},
    time::{SystemTime, UNIX_EPOCH},
};

struct DiscordControlSurface {
    client: DiscordIpcClient,
    project_name: String,
    track_name: String,
    timestamp: Timestamps,
    playstate: PlayState,
    counter: u32,
}

impl DiscordControlSurface {
    fn new() -> Self {
        let reaper = Reaper::get().medium_reaper();
        let mut client = DiscordIpcClient::new("1182008408354865183").unwrap();
        client.connect().unwrap_or_else(|op| {
            reaper.show_console_msg(format!(
                "Unable to establish conenction to Discord RPC: {}",
                op
            ))
        });

        let start = SystemTime::now();
        let ts = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Self {
            client,
            project_name: String::new(),
            track_name: String::new(),
            counter: 0,
            playstate: PlayState {
                is_paused: false,
                is_playing: false,
                is_recording: false,
            },
            timestamp: Timestamps::new().start(ts.as_secs() as i64),
        }
    }

    fn update_activity(&mut self) {
        let reaper = Reaper::get().medium_reaper();
        let trackname = format!("Editing {}", self.track_name);
        let project = format!("Project {}", self.project_name);
        let version = &format!("REAPER v{}", reaper.get_app_version());

        let transport_state = if self.playstate.is_playing {
            "transport-playing"
        } else if self.playstate.is_recording {
            "transport-recording"
        } else if self.playstate.is_paused {
            "transport-pause"
        } else {
            "transport-stopped"
        };
        let assets = Assets::new().large_image("reaper").large_text(version).small_image(transport_state);
        match self.client.set_activity(
            activity::Activity::new()
                .details(&trackname)
                .state(&project)
                .assets(assets)
                .timestamps(self.timestamp.clone()),
        ) {
            // just shut the fuck up
            _ => {}
        }
    }
}

// stub
impl Debug for DiscordControlSurface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MyControlSurface").finish()
    }
}

impl ControlSurface for DiscordControlSurface {
    fn run(&mut self) {
        if self.counter > 30 {
            let project = Reaper::get().current_project();
            self.playstate = project.play_state();
            self.project_name = if let Some(name) = project.file() {
                name.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            } else {
                String::from("<unsaved project>")
            };

            self.track_name = if let Some(track) = project
                .selected_tracks(reaper_medium::MasterTrackBehavior::ExcludeMasterTrack)
                .nth(0)
            {
                track.name().unwrap_or_default().to_string()
            } else {
                String::from("none")
            };

            self.update_activity();

            self.counter = 0;
        } else {
            self.counter += 1;
        }
    }
}

#[reaper_extension_plugin(
    name = "Discord Rich Presence",
    support_email_address = "laineprikol@gmail.com"
)]
fn plugin_main() -> Result<(), Box<dyn Error>> {
    let mut session = Reaper::get().medium_session();

    session
        .plugin_register_add_csurf_inst(Box::new(DiscordControlSurface::new()))
        .unwrap();

    Ok(())
}
