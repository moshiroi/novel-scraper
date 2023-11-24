# syntax=docker/dockerfile:1
# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/engine/reference/builder/
FROM selenium/standalone-chrome:119.0.6045.123-chromedriver-119.0.6045.105 as base

USER root

RUN apt-get update && apt-get install -y python3-pip

ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1

WORKDIR /app

COPY . .

RUN python3 -m pip install poetry

# Install dependencies using Poetry
RUN poetry install

# Expose the port that the application listens on.
EXPOSE 8000

# Run the application.
CMD poetry run python3 main.py