PKG=rollenspielsache-svc
VERSION=0.1.2
LOCALTAG=$(PKG):$(VERSION)
REMOTE=deciduously0
REMOTETAG=$(REMOTE)/$(LOCALTAG)

dev:
	systemfd --no-pid -s http::3000 -- cargo watch -x run

docker:
	docker build -t $(LOCALTAG) .

release: docker
	docker tag $(LOCALTAG) $(REMOTETAG)
	docker push $(REMOTETAG)

.PHONY: dev docker release