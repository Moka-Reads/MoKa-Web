LOCAL_ADDY = http://0.0.0.0:8000
REMOTE_ADDY = https://moka-reads.mkproj.com/

profiling_local:
	locust -f profiling.py --host=$(LOCAL_ADDY)
	echo "Profiling at http://localhost:8089/"

profiling_remote:
	locust -f profiling.py --host=$(REMOTE_ADDY)
	echo "Profiling at http://localhost:8089/"