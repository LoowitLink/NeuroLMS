# Use Node.js official image for building
FROM node:18-alpine as builder
WORKDIR /usr/src/app
COPY package*.json ./
RUN npm install --legacy-peer-deps
COPY . .
RUN npm run build

# Run the Next.js application
FROM node:18-alpine

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app ./
EXPOSE 3000

# Start the Next.js app using 'npm run start' on port 80
CMD ["npm", "run", "start"]