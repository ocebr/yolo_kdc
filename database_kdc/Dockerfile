FROM postgres
RUN mkdir /app
ADD ./migrations/yolo_bundle.sql /app
ADD ./.env /app
ADD ./init-db.sh /app
WORKDIR /app
RUN chmod +x ./init-db.sh


ENTRYPOINT [ "./init-db.sh" ]
