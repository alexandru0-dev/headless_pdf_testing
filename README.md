Repo for testing libraries for webdriver for Rust, for headless mode and with the scope of pdf generation with async support.
This project uses axum as uses Tokio and integrates with the project with a REST API which can be benchmarked and easy to interface with.

```sh
# Run webdriver proxy before run project

# if you want to use firefox, run:
geckodriver

# or chromium instead, run:
chromedriver --port=4444
```

## How it parallelize webdriver:
- Open new tab for each page to convert to pdf
- Switch to tab, convert current tab to pdf (there is no other way to print another tab then the one you are on), close tab and switch to blank tab
Using mutex/rwlock in both contexts

NEEDS a blank tab, as closing all tabs, closes the windows which closes the connection with webdriver.

