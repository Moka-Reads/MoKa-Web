from locust import HttpUser, task, between, events
import json 
import csv


class RootHandlers(HttpUser):
    wait_time = between(3, 10)

    @task
    def index(self):
        self.client.get("/")

    @task
    def mission(self):
        self.client.get("/mission")

    @task
    def license(self):
        self.client.get("/licenses")

    @task
    def articles(self):
        self.client.get("/articles")

    @task
    def cheatsheets(self):
        self.client.get("/cheatsheets")

    @task 
    def resources(self): 
        f = open('resources/resources.json')
        data = json.load(f)

        for r in data['routes']: 
            self.client.get(r)
    
