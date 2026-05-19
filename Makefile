compile:
	cd src/frontend
	javac -d bin -cp ./open.java
	javac -d bin -cp ./main/App.java
	jar cvf target/L3snik.jar -C bin .
	java -cp target/L3snik.jar open.java

clean:
	cd src/proxy
	rm -r ./target
