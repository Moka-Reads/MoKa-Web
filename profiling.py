from locust import FastHttpUser, task, between
import gevent
import json 

f = open('resources/resources.json')
data = json.load(f)

class RootHandlers(FastHttpUser):
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

        def get_route(url):
            self.client.get(url)

        pool = gevent.pool.Pool()

        for r in data['routes']: 
            pool.spawn(get_route, r)
        pool.join()
            

    @task
    def awesome(self): 
        for p in range(1, 10, 1): 
            self.client.get(f"/awesome/{p}")
    
