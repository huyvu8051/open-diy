# Dockerfile for running Playwright E2E tests
FROM mcr.microsoft.com/playwright:v1.44.1-jammy

WORKDIR /app

# Copy package configurations and install dependencies
COPY package*.json tsconfig.json playwright.config.ts ./
RUN npm ci

# Install Playwright browser engines specifically for the test runner
RUN npx playwright install --with-deps

# Copy the tests directory
COPY tests/ ./tests

ENV BASE_URL=https://shop.opendiy.vn

CMD ["npx", "playwright", "test"]
