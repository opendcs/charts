# OpenDCS Helm Charts

As they are developed this will be the location of organization provided helm charts for OpenDCS applications.

The primary goals are as follows:

- Setup a complete stack of applications: LRGS, RoutingScheduler, CompDepends, CompProc, and the rest_api when in a sufficiently ready state
- Database
  - User provided connection information
  - Cloud Native Postgres Operator to create a fresh database
- Ability to auto scale compdepends, routingscheduler, and compproc as appropriate
- Support for exporting metrics (most likely the prometheus operator, but we're open to suggestions)
- Ability for certain components to be optional, like the LRGS.


Please bear with us as we have only recently started to use tools such as kubernetes and any one using charts once available should consider them experimental at best.
We will label our versions appropriately.
