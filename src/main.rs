fn main() {
    //let resistors = vec![Resistor::new(0.0, (5,7)),
                        //Resistor::new(0.0, (7,6)),
                        //Resistor::new(0.0, (1,4)),
                        //Resistor::new(0.0, (9,8)),
                        //Resistor::new(0.0, (1,5)),
                        //Resistor::new(0.0, (1,3))];
    let resistors = vec![Resistor::new(100.0, (1,2)),
                        Resistor::new(100.0, (1,2)),
                        Resistor::new(100.0, (3,4)),
                        Resistor::new(100.0, (2,3))];
    let r_th = compute_thevenin_resistance(&resistors, (1,4));
    println!("{}", r_th.resistance);
}

fn compute_thevenin_resistance(resistors: &[Resistor], terminals: (u32, u32)) -> Resistor {
    //Might need to rename nodes so that you are computing thevenin resistance for (min_node,
    //max_node)?
    let terminals = (terminals.0.min(terminals.1), terminals.0.max(terminals.1));
    let mut resistors: Vec<Resistor> = resistors.iter().map(|resistor| Resistor::new(resistor.resistance,
                                                            (resistor.nodes.0.min(resistor.nodes.1),
                                                            resistor.nodes.0.max(resistor.nodes.1))))
                                                        .collect();

    while resistors.len() > 1 {
        resistors.sort_by(|a, b| a.nodes.1.cmp(&b.nodes.1));
        resistors.sort_by(|a, b| a.nodes.0.cmp(&b.nodes.0));
        resistors = combine_parallel_resistance(&resistors);
        resistors = combine_series_resistance(&resistors);
    }

    Resistor::new(resistors[0].resistance, resistors[0].nodes)
}

fn combine_parallel_resistance(resistors: &[Resistor]) -> Vec<Resistor> {
    let mut output_resistors = Vec::new();
    let mut consumed_resistors = Vec::new();
    for i in 0..resistors.len() {
        for j in (i+1)..resistors.len() {
            if consumed_resistors.contains(&i) || consumed_resistors.contains(&j) {
                continue;
            }
            if resistors[i].nodes == resistors[j].nodes {
                let equivalent_resistance = resistors[i].resistance * resistors[j].resistance
                                            / (resistors[i].resistance + resistors[j].resistance);
                output_resistors.push(Resistor::new(equivalent_resistance, resistors[i].nodes));
                consumed_resistors.push(i);
                consumed_resistors.push(j);
            }
        }
        if !consumed_resistors.contains(&i) {
            output_resistors.push(Resistor::new(resistors[i].resistance, resistors[i].nodes));
        }
    }
    output_resistors
}

fn combine_series_resistance(resistors: &[Resistor]) -> Vec<Resistor> {
    let mut output_resistors = Vec::new();
    let mut consumed_resistors = Vec::new();
    for i in 0..resistors.len() {
        for j in (i+1)..resistors.len() {
            if consumed_resistors.contains(&i) || consumed_resistors.contains(&j) {
                continue;
            }
            if (resistors[i].nodes.1 == resistors[j].nodes.0)
                && (resistors.iter()
                            .filter(|r| r.nodes.1 == resistors[i].nodes.1)
                            .count() == 1)
                && (resistors.iter()
                            .filter(|r| r.nodes.0 == resistors[i].nodes.1)
                            .count() == 1) {
                output_resistors.push(Resistor::new(resistors[i].resistance + resistors[j].resistance,
                                                    (resistors[i].nodes.0, resistors[j].nodes.1)));
                consumed_resistors.push(i);
                consumed_resistors.push(j);
            }
        }
        if !consumed_resistors.contains(&i) {
            output_resistors.push(Resistor::new(resistors[i].resistance,
                                                resistors[i].nodes));
        }
    }
    output_resistors
}

    //for resistor in resistors {
        //println!("{:?}", resistor.nodes);
    //}

    //Now filter out the resistors with nodes outside the range
    //Filter out resistors with nodes.0 less than terminals.0
    //Maybe you can't even do that
    //My thinking was you could filter out resistors outside the range
    //to get the subcircuit you are interested in
    //Then find parallel equivalent to end up with unique resistors
    //Then all the non-unique parts of the tuple correspond to
    //different branches, and I don't yet know how to best combine these
    //branches, but maybe if you keep another list of resistors sorted by
    //nodes.1 instead of nodes.0, I think that would work because then I
    //think you just take the first element as the one you want
    //Stable sort is important

    //Ok, I think I have an algorithm that will work:
    //Rename the 2 nodes you are testing as smallest and highest nodes.
    //Sort nodes.1.
    //Stable sort nodes.0.
    //Apply parallel resistance rule so that you have unique elements in the list.
    //Maintain a separate list of the nodes sorted in the opposite way.
    //The first element of both lists are guaranteed to connect in series,
    //because the upper node of the first element is the minimum of both the list of
    //upper nodes and the list of lower nodes, hence the global minimum.
    //So once you have consumed this element, re-sort and go back to step 1.
    //Note that it might not be as simple as adding in series, because there might
    //be multiple branches, so you are adding a parallel resistance in series. But you
    //can handle that case by spawning new parallel elements and pairing each with
    //one of the branches, then you can add them in series because no current will flow
    //between them so you can remove the wires joining the newly spawned parallel elements.
    //The equivalent resistance of the spawned parallel elements will obviously need
    //to match the original resistance of the resistor you are converting into
    //multiple spawned resistors.
    //Actually I am pretty sure I am wrong about the validity of spawning new resistors
    //because current would flow along the wires connecting them, so you cannot erase
    //the interconnections. So back to the drawing board.
    //Probably keep most of the algorithm, but use recursion to find equivalent
    //resistances until you end up with series resistances which you can simply sum
    //together.

    //I think a better solution, that I can more easily verify is correct, is to
    //successively reduce the number of resistors by either applying a parallel
    //simplification or series simplification.
    //Because that is all you can do when simplifying a circuit by hand anyway.

    //let mut thevenin_resistance = 0.0;
    //let mut idx = 0;
    //for (i, resistor) in resistors.iter().enumerate() {
        //if resistor.nodes.0 == terminals.0 {
            //thevenin_resistance = resistor.resistance;
            //idx = i + 1;
            //break;
        //}
    //}
    //for j in idx..resistors.len() {
        //if 
    //}


struct Resistor {
    resistance: f64,
    nodes: (u32, u32),
}

impl Resistor {
    fn new(resistance: f64, nodes: (u32, u32)) -> Self {
        Resistor {
            resistance,
            nodes,
        }
    }
}
