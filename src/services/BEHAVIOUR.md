# SERVICE BEHAVIOUR

This is a small file specifying the behaviour of services.

Services have the following methods, which should be run accordingly:

- init
  - runs once, if it fails, the service will be aborted.

- main
  - loops, if it fails, the service will run the catch method with the propagated error; if it succeeds, the repeat method will run.

- repeat
  - this method runs after the main method if no errors were thrown. If it fails, the service will run the catch method with the propagated error.

- catch
  - the catch method runs if any errors were thrown on the main or the repeat methods.

- abort
  - the abort method runs if the catch method fails and has the propagated error.

- init
  - main
    - repeat
  - catch
- abort
