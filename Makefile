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
