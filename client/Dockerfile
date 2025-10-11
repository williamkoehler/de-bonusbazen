# ---- Build stage ----
FROM node:24 AS builder

WORKDIR /app

# Install dependencies
COPY package*.json ./
RUN npm install

# Copy project files
COPY . .

# Build app
RUN npm run build --production

# ---- Runtime stage ----
FROM nginx:alpine

COPY --from=builder /app/dist/de-bonusbazen/browser /usr/share/nginx/html

COPY nginx.conf /etc/nginx/templates/default.conf.template

EXPOSE 80

CMD envsubst '$BACKEND_HOST $BACKEND_PORT' < /etc/nginx/templates/default.conf.template > /etc/nginx/conf.d/default.conf \
    && nginx -g 'daemon off;'