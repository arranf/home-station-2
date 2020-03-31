# service

Service contains the basic foundation for `TimeClient` and `WeatherClient` which can be used to request data from concrete implementations of each client type.

It also includes `Poller` which is to implement fetching from each client on an interval basis.
