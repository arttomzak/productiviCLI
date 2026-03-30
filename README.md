## productiviCLI

productiviCLI is a simple CLI tool that lets you track your time spent 
on anything!

This tool currently writes session data to a postgreSQL database that I 
personally run within a docker container locally, however I plan to 
deploy a database for any of my devices to be able to connect to down
the road, and create a cool little waybar module that shows you the name 
of the task you're tracking along with the current running time!

Also in the plans is a simple dashboard hooked up to said deployed database
showcasing some cool queries based on your data

Even further in the future I'd like to allow others to either write to my deployed
database with accounts on the website or be able to hook up their own databases
to the website to run the same queries on their data


# Installation

Clone the repository

Run the migrations within migrations/ within your docker container
to replicate the postgreSQL database.

Create an .env file mimicking the format within .env.example for the DB link
(if running your database locally on a docker container) 

Bang!
