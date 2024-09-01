

# rsTrade

The objective is to do a cross-platform trading center.

**!!! I'm not a developer, and it's my first try with rust. But I try to be the more clean as possible. :blush:**

## The idea

An application based on egui with egui_tiles for the windows management.

Each functionality will be inside a plugin
 * Panes : frontend of a functionality ( connections / accounts / orders / positions / chart / ... )
 * With node graph :
   * Data provider : collect data ( market data / news data / ... )
   * Data Calculation : all work to calculate / aggregate / analyze / ..., generic with inline code (python/lua/?)
   * Actions : create / modify / copy orders / positions, ...., generic with inline code (python/lua/?
 * Messaging between components : add messages to the bus to put / get data / events / ...
 * ...

Everything can change, as the platform evolves and as I evolve :blush:.

## Objectives

 * [x] Do the minimum to support egui_tiles & plugin loading
 * [ ] Implement bus communication between all components (use Rithmic WS Order routing as input data)
 * [ ] Add copy trading based on rules through node graph
 * [ ] Add basic DOM and Time & Sales
 * [ ] Add basic chart
 * [ ] ...