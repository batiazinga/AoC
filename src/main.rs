use aoc2023::beam_tracer::Contraption;

fn main() {
    let c = Contraption::parse("..\\.\n....\n..-.");
    c.trace_beam();
}
