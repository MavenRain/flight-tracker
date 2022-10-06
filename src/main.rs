mod airport_code;

use {
    self::airport_code::AirportCode,
    actix_web::{post, web::Json, App, HttpServer, Responder},
    std::io::Result,
};

type Flight = [AirportCode; 2];

fn merge_flights(flights: [Flight; 2], location: AirportCode) -> Flight {
    [
        if flights[0][0] == location {
            flights[0][1]
        } else {
            flights[0][0]
        },
        if flights[1][0] == location {
            flights[1][1]
        } else {
            flights[1][0]
        },
    ]
}

#[allow(clippy::clone_on_copy)]
fn find_flights_to_merge(flights: &[Flight], location: AirportCode) -> Option<[Flight; 2]> {
    let mut flights_to_merge = flights
        .iter()
        .filter(|flight| flight.contains(&location))
        .take(2);
    flights_to_merge.next().and_then(|first_flight| {
        flights_to_merge
            .next()
            .map(|second_flight| [first_flight.clone(), second_flight.clone()])
    })
}

fn find_the_first_common_location(flights: &[Flight]) -> Option<AirportCode> {
    flights
        .iter()
        .fold(vec![], |mut locations, flight| {
            locations.extend(flight);
            locations
        })
        .iter()
        .find(|location| {
            flights
                .iter()
                .filter(|&flight| flight.contains(location))
                .count()
                == 2
        })
        .cloned()
}

fn calculate_flights(flights: Vec<Flight>) -> Vec<Flight> {
    let default = flights.clone();
    find_the_first_common_location(flights.as_slice())
        .and_then(|location| {
            find_flights_to_merge(flights.as_slice(), location).map(|flights_to_merge| {
                calculate_flights({
                    let mut filtered_flights = flights
                        .into_iter()
                        .filter(|flight| !flights_to_merge.contains(flight))
                        .collect::<Vec<_>>();
                    filtered_flights.extend(&[merge_flights(flights_to_merge, location)]);
                    filtered_flights
                })
            })
        })
        .unwrap_or(default)
}

#[post("/calculate")]
async fn calculate(Json(flights): Json<Vec<Flight>>) -> impl Responder {
    Json(calculate_flights(flights))
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(move || App::new().service(calculate))
        .bind("127.0.0.1:8080")
        .map(|server| server.run())?
        .await
}

#[cfg(test)]
mod test {
    use super::{airport_code::AirportCode, calculate_flights};

    #[test]
    fn calculate_multi_leg_single_flight() {
        use AirportCode::{ATL, EWR, GSO, IND, SFO};

        assert_eq!(
            calculate_flights(vec![[IND, EWR], [SFO, ATL], [GSO, IND], [ATL, GSO]]),
            vec![[EWR, SFO]]
        )
    }
}
