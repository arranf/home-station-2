# ui-framework

`ui-framework` glues together Conrod, Glium, Winit, and a `Scheduler`.

## System

`System::start()` weaves together functions from `bootstrap` to produce a UI that appears.

It:

* Finds the asset folder
* Initialises glium
* Initialises conrod
* Initialises the scheduler
* Initialises the texture controller
* Initialises the navigation controller
* Navigates to the starting screen
* Starts the event loop

## Scheduler

The scheduler is mostly half baked. It's designed (it appears) to be a replacement for the Token Poller.
The scheduler listens to inputs (key presses, mouse input, etc.) and responds to them.
The scheduler also creates a task dispatcher, which is currently not used, and a process task loop which is not fleshed out.
