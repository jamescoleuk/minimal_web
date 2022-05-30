# Forecaster

This is a demonstration or proof-of-concept of a minimal web using some patterns and tech that we all like at the moment. We're talking:

1. Rust
2. HTMX
3. Server-side rendering
4. Minimal CSS
5. SQLite

I've worked exclusively on SPAs for the last decade, so a lot of this is me remembering how these things work, and figuring out how to organise an app like this.


## File organisation
Dirs and files for UI have a 1-to-1 mapping. E.g.:
* `src/forecasts/ui/list.rs` maps to `templates/forecasts/_list.html`

Askama is great for detecting drift between the template and the code, but the templates aren't stored next to the code. This is a bit crap, because you've got to look in two places. Code that changes together should live together. I've looked at libraries that provide macros to build HTML, but that is a too insuffrably rubbish way of doing HTML. In the absence of JSX for Rust the best I can do is keep the directory structures aligned by convention.





## Routes
1. GET    / - redirect to /forecast
2. GET    /forecast/ show a list of all forecasts
3. GET    /forecast/create show the forecast creation page
4. POST   /forecast/create - create the new forecast
4. GET    /forecast/1 - show forecast with id=1. Render what's been set up so far.
4. DELETE /forecast/1 - delete forecast with id=1
4. POST   /forecast/1 - update forecast with id=1