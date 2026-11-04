#!/usr/bin/env node
"use strict";

/**
 * Validate repository-owned Markdown links and JSON syntax without third-party
 * dependencies. YAML syntax is checked with the Ruby standard-library parser
 * available on the pinned Ubuntu runner.
 *
 * This is a repository-integrity check. It does not validate external URLs,
 * Markdown rendering, JSON Schema semantics, or the content of research claims.
 */

const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const ROOT = path.resolve(__dirname, "..");
const IGNORED_DIRECTORIES = new Set([
  ".git",
  "target",
  "node_modules",
  ".venv",
  "vendor",
]);

function walk(directory, output = []) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    if (entry.isSymbolicLink()) continue;

    const fullPath = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      if (!IGNORED_DIRECTORIES.has(entry.name)) walk(fullPath, output);
      continue;
    }

    if (!entry.isFile()) continue;

    const extension = path.extname(entry.name).toLowerCase();
    if ([".md", ".json", ".yml", ".yaml"].includes(extension)) {
      output.push(fullPath);
    }
  }

  return output;
}

function relativeName(filePath) {
  return path.relative(ROOT, filePath).split(path.sep).join("/");
}

function stripFencedCode(markdown) {
  return markdown
    .replace(/\x60{3,}[\s\S]*?\x60{3,}/g, "")
    .replace(/~{3,}[\s\S]*?~{3,}/g, "");
}

function extractInlineLinks(markdown) {
  const links = [];
  const pattern =
    /!?\[[^\]]*\]\(\s*(?:<([^>\n]+)>|([^\s)]+))(?:\s+(?:"[^"]*"|'[^']*'|\([^)]*\)))?\s*\)/g;

  for (const match of markdown.matchAll(pattern)) {
    links.push(match[1] || match[2]);
  }

  return links;
}

function resolveRepositoryLink(sourceFile, rawHref) {
  if (!rawHref || rawHref.startsWith("#") || rawHref.startsWith("//")) {
    return null;
  }

  if (/^[a-z][a-z0-9+.-]*:/i.test(rawHref)) {
    return null;
  }

  let decoded;
  try {
    decoded = decodeURIComponent(rawHref);
  } catch {
    throw new Error(
      `Invalid percent-encoding in link from ${relativeName(sourceFile)}: ${rawHref}`,
    );
  }

  const withoutFragment = decoded.split("#", 1)[0];
  const withoutQuery = withoutFragment.split("?", 1)[0];
  if (!withoutQuery) return null;

  const target = withoutQuery.startsWith("/")
    ? path.resolve(ROOT, withoutQuery.slice(1))
    : path.resolve(path.dirname(sourceFile), withoutQuery);

  const relative = path.relative(ROOT, target);
  if (relative === ".." || relative.startsWith(`..${path.sep}`) || path.isAbsolute(relative)) {
    throw new Error(
      `Repository link escapes the repository root: ${relativeName(sourceFile)} -> ${rawHref}`,
    );
  }

  return target;
}

function validateMarkdown(files) {
  const errors = [];

  for (const file of files.filter((item) => path.extname(item).toLowerCase() === ".md")) {
    const markdown = stripFencedCode(fs.readFileSync(file, "utf8"));

    for (const href of extractInlineLinks(markdown)) {
      let target;
      try {
        target = resolveRepositoryLink(file, href);
      } catch (error) {
        errors.push(error.message);
        continue;
      }

      if (!target) continue;

      let exists = fs.existsSync(target);
      if (exists && fs.statSync(target).isDirectory()) {
        exists =
          fs.existsSync(path.join(target, "README.md")) ||
          fs.existsSync(path.join(target, "index.md"));
      }

      if (!exists) {
        errors.push(
          `Broken local link: ${relativeName(file)} -> ${href}`,
        );
      }
    }
  }

  return errors;
}

function validateJson(files) {
  const errors = [];

  for (const file of files.filter((item) => path.extname(item).toLowerCase() === ".json")) {
    try {
      JSON.parse(fs.readFileSync(file, "utf8"));
    } catch (error) {
      errors.push(
        `Invalid JSON: ${relativeName(file)}: ${error.message}`,
      );
    }
  }

  return errors;
}

function validateYaml(files) {
  const yamlFiles = files.filter((item) =>
    [".yml", ".yaml"].includes(path.extname(item).toLowerCase()),
  );

  if (yamlFiles.length === 0) return [];

  const ruby = spawnSync(
    "ruby",
    [
      "-e",
      'require "yaml"; ARGV.each { |file| YAML.parse_file(file) }',
      ...yamlFiles,
    ],
    { cwd: ROOT, encoding: "utf8" },
  );

  if (ruby.error) {
    return [`Unable to run Ruby YAML parser: ${ruby.error.message}`];
  }

  if (ruby.status !== 0) {
    return [
      "YAML parser reported an error:",
      ruby.stdout || "",
      ruby.stderr || "",
    ].filter(Boolean);
  }

  return [];
}

function main() {
  const files = walk(ROOT);
  const errors = [
    ...validateMarkdown(files),
    ...validateJson(files),
    ...validateYaml(files),
  ];

  if (errors.length > 0) {
    console.error("Repository policy validation FAILED:");
    for (const error of errors) console.error(`- ${error}`);
    process.exitCode = 1;
    return;
  }

  const markdownCount = files.filter((file) => file.endsWith(".md")).length;
  const jsonCount = files.filter((file) => file.endsWith(".json")).length;
  const yamlCount = files.filter((file) => /\.ya?ml$/i.test(file)).length;

  console.log("Repository policy validation PASS");
  console.log(`Markdown files checked: ${markdownCount}`);
  console.log(`JSON files parsed: ${jsonCount}`);
  console.log(`YAML files parsed: ${yamlCount}`);
}

main();
