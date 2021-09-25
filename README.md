# abm-wildfire
## A simple Rust implementation of the common Fire agent-based model

For a more detailed explanaion of the Fire model, see the [NetLogo Example](http://www.netlogoweb.org/launch#http://ccl.northwestern.edu/netlogo/models/models/Sample%20Models/Earth%20Science/Fire.nlogo).

This repo is already compiled. Run `target/release/abm_wildfire` to run the binary.

### Program behavior

After taking a console input for the density of the forest, the program prints a randomly generated forest to the console. After a moment, the furthest left-hand column ignites, and the fire spreads through the forest.

The fire spreads from a burning tree to neighboring trees in up, down, left, and right directions (not diagonally). Trees have the following states:
- Clear: Represented by an empty space - There is no tree here
- Living: `T` - The beginning state of each tree
- Ignited: `#` - A tree that has been ignited by a neighbor (or on initiation)
- Burning: `%` - A tree that is burning and can ignite neighboring trees
- Burned: `X` - A tree that has finished burning

When no trees are left burning, the program prints the % of the forest that has been burned.

### Model behavior

This classic model illustrates well the concept of tipping points. The probability that the fire will reach the right side of the forest is relatively low for forest densities below 59% but is relatively high for densities above 59%.

For a more detailed explanaion of the Fire model, see the [NetLogo Example](http://www.netlogoweb.org/launch#http://ccl.northwestern.edu/netlogo/models/models/Sample%20Models/Earth%20Science/Fire.nlogo).