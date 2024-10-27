FROM nginx:alpine

COPY index.html /usr/share/nginx/html/

RUN sed -i 's/80/8080/g' /etc/nginx/conf.d/default.conf

EXPOSE 8080