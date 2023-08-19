LOCAL_ADDY = http://0.0.0.0:8000
REMOTE_ADDY = https://moka-reads.mkproj.com/
ULIMIT = 65535

profiling_local:
	ulimit -n $(ULIMIT)
	locust -f profiling.py --host=$(LOCAL_ADDY)
	echo "Profiling at http://localhost:8089/"

profiling_remote:
	ulimit -n $(ULIMIT)
	locust -f profiling.py --host=$(REMOTE_ADDY)
	echo "Profiling at http://localhost:8089/"

profiling_headless: 
	ulimit -n $(ULIMIT)
	locust -f profiling.py --headless --host $(LOCAL_ADDY) -u 100000 -r 100 --run-time 1h > logs/log.txt --html profiling_reports/$(shell date +'%Y-%m-%d').html

build_docki: 
	docker build -t moka-web:latest . 
start_container: 
	docker run -d --name moka_reads -p 8000:8000 moka-web:latest  
stop_container: 
	docker stop moka_reads 
rm_container: 
	docker rm moka_reads
exec_it: 
	docker exec -it moka_reads /bin/sh 
