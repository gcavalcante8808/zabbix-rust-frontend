Zabbix-Rust-Frontend
--------------------

This project aims to be a complete implementation of the Zabbix API and the Zabbix Frontend, that today is writen in PHP.

The main goals of this project is to remmedy the classic flaws, as follows:

 * Provide a fast and, if possible, escalable version of the API;
 * Simplify the deploy and drop the needs of a PHP interpreter;
 * Provide a modern Frontend that rellies just on the API to get the monitoring information;
 * Provide a Rest API for introspection of the data;
 * Provide a Compatibility Layer to the JSONRPC Api.

RoadMap
-------

The RoadMap is extensive, but these are the Major Milestones:

 * Define the solution that will be used to connect to the databases (Is an ORM a dream? Or it can be used in some places?)
 * Define the solution that will be used in the frontend (Angular, Polymer, Pure HTML, NodeJS pig + JS, etc ...);
 * Define the architecture that allow us to route the Rest and JsonRPC calls to their right places;
 * Move apiinfo into lib and then start the real development;
 * Verify the Coverage and implement the CI routines;
 * Implement all "Get" options for the API methods;
 * Implement all "Delete" options for the API methods;
 * Implement all "Update" options for the API methods;

Obstacles
---------

These are some obstacles that we need think about:

 * The apiinfo.version have backward compatibility problems (the auth field must be present on requests on the version 2.X, but cannot be at 3.X); Do we have to implement such behaviour? How much are we adherent to the old API?
 * There is no API to get the Status of a Zabbix Server (VPS, ZBX Server Is Running, Strats for hosts,items, triggers, users ...). Should we Implement One? If yes, Do we need to provide a JSON RPC for it as well?
 * Get more people to help and enhance the project (even more ideas and implementations would be awesome).

Implementation Status
---------------------

This is the follow implementation status of the project:

Nothing at moment. The ApiInfo will respond accordingly, but will not return errors about the auth param (which is the current behaviour).
