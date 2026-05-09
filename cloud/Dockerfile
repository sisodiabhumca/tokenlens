# syntax=docker/dockerfile:1.6
FROM node:20-alpine AS deps
WORKDIR /app
COPY cloud/package.json ./cloud/package.json
COPY package.json pnpm-workspace.yaml ./
RUN corepack enable && corepack prepare pnpm@9 --activate \
 && pnpm install --filter ./cloud --no-frozen-lockfile

FROM node:20-alpine AS build
WORKDIR /app
COPY --from=deps /app /app
COPY cloud ./cloud
RUN corepack enable && pnpm --filter ./cloud build

FROM node:20-alpine AS run
WORKDIR /app
ENV NODE_ENV=production PORT=3000
COPY --from=build /app /app
EXPOSE 3000
CMD ["node", "cloud/node_modules/next/dist/bin/next", "start", "cloud", "-p", "3000"]
