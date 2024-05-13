use std::fs::read_to_string;

use sysinfo::{ Components, Disks, System, RefreshKind, ProcessRefreshKind};

use anathema::runtime::Runtime;
use anathema::templates::Document;
use anathema::backend::tui::TuiBackend;
use anathema::widgets::components::Component;
use anathema::state::*;
use anathema::widgets::components::events::{KeyCode, KeyEvent, MouseEvent, MouseState, MouseButton, KeyState};
use anathema::widgets::Elements;

#[derive(State)]
struct RunningProcessState {
    processes: Value<List<String>>, 
}

impl RunningProcessState {
    pub fn new() -> Self {
        Self {
            processes: List::empty(),
        }
    }
}

struct ProcessComponent;

impl Component for ProcessComponent {
    type State = RunningProcessState;
    type Message = ();
    
    fn on_mouse(
        &mut self,
        mouse: MouseEvent,
        state: Option<&mut Self::State>,
        elements: Elements<'_, '_>,
    ) {
        let state = state.unwrap();

        let mut processes = &mut state.processes;

        // Scrolling the list up and down
        if let MouseState::ScrollDown = mouse.state {
            let val = processes.pop_front().unwrap();
            processes.push_back(val);
        }
    }
}

fn main() {
    let mut sys = System::new();

    sys.refresh_specifics (
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );

    let template = read_to_string("template.aml").unwrap();

    let mut doc = Document::new(template);

    let mut backend = TuiBackend::builder()
        //.enable_alt_screen()
        .enable_mouse()
        .finish()
        .unwrap();

    let mut running_process_state = RunningProcessState::new();

    
    for (_, process) in sys.processes() {
        running_process_state.processes.push_front(process.name().to_string());
    }
    


    let component_id = doc.add_component("process-comp", "
    vstack
        for val in processes
            text val
    ");

    let mut runtime = Runtime::new(doc, backend).unwrap();
    
    runtime.register_component(component_id, ProcessComponent, running_process_state);

    runtime.run(); 
}
