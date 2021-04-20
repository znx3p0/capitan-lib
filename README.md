# CAPITAN

## Service library

Capitan allows you to create and configure services and supervisor trees through reactors and services.

There are various concepts you have to understand before building supervisor trees with capitan, since capitan's take
on supervisor trees is different from Erlang's take.

Concepts:

- Reactors
  - Isolated
  - Shared
  - Dynamic
- Services
  - Isolated
  - Shared
  - Dynamic

### Services

Services are structures that implement the service trait.
Services can be isolated or shared, but they can also be dynamic.

- Isolated
  - Isolated services offer a mutable reference to self, that's why they cannot safely be accessed from outside. They are useful for services which operate on singletons, or services that don't need any interaction.

- Shared
  - Shared services offer a reference to self. They are useful for services which need outside interaction, such as monitoring services.

- Dynamic
  - Dynamic services can be either isolated or shared services, the difference between dynamic and normal services is that dynamic services can be run on dynamic reactors.
  Normal reactors have to run the same exact service, so running different services on the same reactor requires the reactor to be dynamic.

### Reactors

Reactors are structures which hold and run services.

- Isolated
  - Isolated reactors run isolated services. They do not offer outside access to their services.

- Shared
  - Shared reactors run shared services. They offer outside access to their inner services.

- Dynamic
  - Dynamic reactors run dynamic services. They can either be dynamic shared services or isolated shared services, but never both.
