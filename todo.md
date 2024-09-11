## for "json" file (done)
- add an id system (done)
- add an app name for each credential (done)

## for egui front end (done)
- another box for application name (done)
- better visuals to display credentials (done)
- maybe a confirmation to display credentials (should be a checkbox i think so its toggleable) (done)
- delete button (done)
- fix credentials not live updating on logins.txt rewrite (done)

## for backend
- add cbc mode of operation (done)
- caclulate id for each credential (done)
- make sure each field is filled out before submitting credentials (done)

## greeter
- make sha256 verification username and password
- transition from greeter to main app

## other
- make a quick program that sets up on first use
    - generates database (or resets)
    - generates IV
    - generates key
    - formats login.txt so it can be used bin egui.rs
        - clone tmp.txt (rename this)
        - encrypt and save as login.txt
