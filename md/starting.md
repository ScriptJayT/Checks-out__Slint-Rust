# A Rust application

After viewing the video tutorial by `Travis Media` about using Rust and Slint to build a desktop application, I got excited to try it out.
So after following Travis's tutorial to set up the base code I got the Rust and Slint documentation ready and started tinkering.

## Borrow trouble

As I am new to Rust it took a bit of trying and reading to get used to the borow checker and find a way that worked. For example to get the application to listen say a button click you use:

<small>src/main.rs</small>

```rust
    let ui: AppWindow = AppWindow::new()?;
    let ui_handle:Weak<AppWindow> = ui.as_weak();
    ui.on_button_click(move || {
        // your stuff
    });
```

and more often than not you use the `ui_handle` variable in the closure to send data back to the UI. Like here we set the ui property `feedback_out` to a string value.

<small>src/main.rs</small>

```rust
    ui.on_button_click(move || {
        let ui: AppWindow = ui_handle.unwrap();
        ui.set_feedback_out("myfeedback".into());
    });
```

But this means that, according to the borrow checker, after the closure the `ui_handle` variable is no longer accessible.
So listening to a second button click would not realy work.

<small>src/main.rs</small>

```rust
    let ui: AppWindow = AppWindow::new()?;
    
    let ui_handle:Weak<AppWindow> = ui.as_weak();
    ui.on_button_click(move || {
        let ui: AppWindow = ui_handle.unwrap();
        ui.set_feedback_out("my feedback".into());
    });
    ui.on_different_button_click(move || {
        let ui: AppWindow = ui_handle.unwrap();
        ui.set_feedback_out("my feedback for this button".into());
    });
```

This would work in Javascript, but not in Rust, you'd have to do something like this:

<small>src/main.rs</small>

```rust
    let ui: AppWindow = AppWindow::new()?;
    
    let ui_handle:Weak<AppWindow> = ui.as_weak();
    ui.on_button_click(move || {
        let ui: AppWindow = ui_handle.unwrap();
        ui.set_feedback_out("my feedback".into());
    });
    let ui_handle:Weak<AppWindow> = ui.as_weak();
    ui.on_different_button_click(move || {
        let ui: AppWindow = ui_handle.unwrap();
        ui.set_feedback_out("my feedback for this button".into());
    });
```

But I am almost certain this is not the best way of doing it.

## Slint Components

The second thing I wanted to figure out was how Slint works with custom components. I knew how to pass in callbacks to already existing buttons:

<small>ui/appwindow.slint</small>

```slint
    export component AppWindow inherits Window {
        callback different-button-click()

        Button {
            clicked => {
                different-button-click()
            }
        }
    }
```

As Travis explained, you need to declare the callback using camel-case (which would be translated in rust to `on_`snake_case) and pass it in the clicked property of the button component.
But what if I build a custom components with their own buttons?

```slint
    component Nav {
        VerticalLayout {
            Text {
                text: "my nav";
            }
            Button {
                text: "click me?";

                clicked => {
                    nav-button-click()
                }
            }
        }
    }
```

Using it in the appwindow like this does not work:

```slint
    export component AppWindow inherits Window {
        callback different-button-click()
        callback nav-button-click()

        Button {
            clicked => {
                different-button-click()
            }
        }

        Nav {}       
    }
```

Slint does not read that far. No, we need to define a new callback property on the `Nav` and then pass our function in it.

```slint
    component Nav {
        callback nav-button-click();

        VerticalLayout {
            Text {
                text: "my nav";
            }
            Button {
                text: "click me?";

                clicked => {
                    nav-button-click()
                }
            }
        }
    }
```

```slint
    export component AppWindow inherits Window {
        callback different-button-click()
        callback nav-click()

        Button {
            clicked => {
                different-button-click()
            }
        }

        Nav {
            nav-button-click { 
                nav-click()
            }
        }       
    }
```

Great! That works, but at the moment the `Nav` component is located in the same file as the `AppWindow`, how to extract that component in a file and, more importantly, include it back in?
Let's say we create a folder and put the component in `components/nav.slint`;
Then we can use following code at the top of the `appwindow.slint` file:

```slint
    import { Nav } from "components/nav.slint";
```

## What follows?

At this point I am scripting away to get a simple password manager to work. The things I believe need figuring out are:

- How to generate a Slint component from Rust data and populate the window at first load
- How to 'navigate' between different pages

So I'll be back after learning how to do that.
