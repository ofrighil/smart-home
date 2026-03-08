use pulse_nyct::service::{Service, Services};
use pulse_nyct::train::Direction;

use mta_indicator::app::{draw_next_three};

fn main() {
    // let station = "34 St-Herald Sq".to_string();
    let station = "47-50 Sts-Rockefeller Ctr".to_string();
    let services: Services = Vec::from([Service::B, Service::D, Service::F, Service::M]).into();
    let direction = Direction::North;

    draw_next_three(station, services, direction);
}
