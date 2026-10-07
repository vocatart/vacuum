/// A location for the vacuum agent to percieve
#[derive(Clone, Copy, Debug, PartialEq)]
enum Location {
    A,
    B,
}

/// Whether a square is clean or dirty
#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    Clean,
    Dirty,
}

/// Actions the agent can take
#[derive(Clone, Copy, Debug, PartialEq)]
enum Action {
    Vacuum,
    Left,
    Right,
}

/// What the agent senses at one time step
type Percept = (Location, Status);

/// Performance measure for scoring each step the agent takes
struct Measure {
    name: &'static str,
    reward: fn(Percept, Action) -> i32,
}

/// Defined performance measures from the assignment
const MEASURES: [Measure; 2] = [
    Measure { name: "+1 point for each location the agent cleans within time T", reward: cleaned_in_time },
    Measure { name: "+1 point for each vacuum per time step, -1 point for each move", reward: vacuum_minus_moves },
];

/// Run an agent action depending on the current perception
///
/// # Arguments
/// `(loc, status)` - The current location and status of the agent
///
/// # Returns
/// `Action` - The next action taken by the agent. Clean@A -> Right, Clean@B -> Left, Dirty -> Vacuum
fn agent((loc, status): Percept) -> Action {
    match (status, loc) {
        (Status::Dirty, _) => Action::Vacuum,
        (Status::Clean, Location::A) => Action::Right,
        (Status::Clean, Location::B) => Action::Left,
    }
}

/// +1 for each location the agent cleans within time T
///
/// # Arguments
/// `(_, status)` - The current status of the agent
/// `action` - The action taken by the agent
///
/// # Returns
/// `i32` - 1 if a dirty square was vacuumed, otherwise 0
fn cleaned_in_time((_, status): Percept, action: Action) -> i32 {
    (status == Status::Dirty && action == Action::Vacuum) as i32
}

/// +1 for each vacuum per time step, -1 for each move
///
/// # Arguments
/// `action` - The action taken by the agent
///
/// # Returns
/// `i32` - 1 for a vacuum, -1 for a move
fn vacuum_minus_moves(_: Percept, action: Action) -> i32 {
    if action == Action::Vacuum { 1 } else { -1 }
}

/// Build one of the two test percept sequences
///
/// # Arguments
/// `n` - Sequence number
///
/// # Returns
/// `Vec<Percept>` - the sequences defined by the assignment
fn sequence(n: u8) -> Vec<Percept> {
    use {Location::*, Status::*};
    match n {
        1 => [(A, Dirty), (B, Dirty)].repeat(50),
        _ => [vec![(A, Dirty), (B, Dirty)], [(A, Clean), (B, Clean)].repeat(49)].concat(),
    }
}

/// Run the agent over a percept sequence and log each step
///
/// # Arguments
/// `measure` - The performance measure to score with
/// `percepts` - The percept sequence fed to the agent
///
/// # Returns
/// `i32` - Final score
fn run(measure: &Measure, percepts: &[Percept]) -> i32 {
    let mut running_score = 0;
    for (i, &p) in percepts.iter().enumerate() {
        let action = agent(p);
        let score = (measure.reward)(p, action);

        running_score += score;

        println!("t={} Percept=[{:?}, {:?}] Action={:?} StepReward={:+} Score={:+}", i + 1, p.0, p.1, action, score, running_score);
    }

    running_score
}

/// Run every measure against every sequence
fn main() {
    for m in &MEASURES {
        for s in 1..=2 {
            println!("Run: Measure=\"{}\"\nSequence={}", m.name, s);
            let score = run(m, &sequence(s));
            println!("FINAL SCORE: {score}\n");
        }
    }
}

#[test]
fn scores() {
    let scores: Vec<i32> = MEASURES.iter()
        .flat_map(|m| (1..=2).map(move |s| run(m, &sequence(s))))
        .collect();

    assert_eq!(sequence(1).len(), 100);
    assert_eq!(sequence(2).len(), 100);
    assert_eq!(scores, [100, 2, 100, -96]);
}
