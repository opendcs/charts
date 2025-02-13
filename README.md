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


# Usage

Documentation TDB. While files exist they haven't been tested and are still mostly the defaults. Unless you are deciding to help with the creation of the chart we
do not recommend trying to use them at this time. There is a lot to work out and until we have have labelled the chart 1.0.0 it should definitely be considered alpha quality
at best.


# Known TODO:

## LRGS


### Storage
The lrgs requires storage. currently some sort of filesystem. There are active projects to use object storage or a database, but a filesystem it required at this time.
The "XML" database is not truly XML (no root tags) and append only so NFS *should* be fine. 

For replicas we recommend 1 and are going to force the chart to that for now. However it can make sense to have individual LRGS instances to provide some redundancy.

### Credentials

Given the age of the software there's some oddities in credential management, and the user accounts are stored on it's permanent volume. May work out a way to have secrets
always used to handle changes.

### Interface

Currently the DDS protocol will need to be exposed raw, there is work on a web app and api that will be integrated as soon as possible.

## Database

Need to make a sidecar container to handle initializing the database. Will also need to setup integration with postgres chart and the Cloud Native Postgres Operator.


## REST API and Web App

The rest api and web app is undergoing some major development. Once it's even remotely pratical it will be integrated into the chart.

## Monitoring/Logging

Since the existing thick client applications for monitoring will not be practical, a container to aggregate the application logs and view them will likely be created.
Additionally we are planning to expose metrics in standard formats and will need to create the appropriate end points and integrations.
