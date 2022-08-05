FROM nginx:alpine

#------------
# Build the app
#------------
COPY ./dist /usr/share/nginx/html
