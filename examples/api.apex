#!/usr/bin/env apex
# ========================================
# WEB & API DEVELOPMENT
# Building RESTful services in Apex
# ========================================

import std.http
import std.json
import std.string

# Data models
class User:
    id: i32
    username: str
    email: str
    created_at: str
    
    fn new(id: i32, username: str, email: str):
        return {
            id: id,
            username: username,
            email: email,
            created_at: clock()
        }
    
    fn to_json():
        return {
            id: this.id,
            username: this.username,
            email: this.email,
            created_at: this.created_at
        }

class Post:
    id: i32
    title: str
    content: str
    author_id: i32
    likes: i32
    
    fn new(id: i32, title: str, content: str, author_id: i32):
        return {
            id: id,
            title: title,
            content: content,
            author_id: author_id,
            likes: 0
        }
    
    fn like():
        this.likes = this.likes + 1
        return this.likes
    
    fn to_json():
        return {
            id: this.id,
            title: this.title,
            content: this.content,
            author_id: this.author_id,
            likes: this.likes
        }

# In-memory database (for demo)
mut users = [
    User.new(1, "alice", "alice@example.com"),
    User.new(2, "bob", "bob@example.com"),
    User.new(3, "charlie", "charlie@example.com")
]

mut posts = [
    Post.new(1, "My First Post", "Hello Apex!", 1),
    Post.new(2, "Learning Functional Programming", "Apex rocks!", 2),
    Post.new(3, "Web Development in Apex", "Building APIs...", 1)
]

# API Handlers

fn get_users() -> {}:
    """GET /users - List all users."""
    let user_list = map(users, fn(u) => u.to_json())
    return {
        status: 200,
        data: user_list,
        message: "Users retrieved successfully"
    }

fn get_user(id: i32) -> {}:
    """GET /users/:id - Get specific user."""
    for user in users:
        if user.id == id:
            return {
                status: 200,
                data: user.to_json(),
                message: "User found"
            }
    
    return {
        status: 404,
        error: "User not found"
    }

fn create_user(username: str, email: str) -> {}:
    """POST /users - Create new user."""
    let new_id = 1 + reduce(users, 0, fn(acc, u) => max(acc, u.id))
    let user = User.new(new_id, username, email)
    users.push(user)
    
    return {
        status: 201,
        data: user.to_json(),
        message: "User created successfully"
    }

fn get_posts() -> {}:
    """GET /posts - List all posts."""
    let post_list = map(posts, fn(p) => p.to_json())
    return {
        status: 200,
        data: post_list,
        message: "Posts retrieved successfully"
    }

fn get_user_posts(user_id: i32) -> {}:
    """GET /users/:id/posts - Get user's posts."""
    let user_posts = filter(posts, fn(p) => p.author_id == user_id)
    let post_list = map(user_posts, fn(p) => p.to_json())
    
    return {
        status: 200,
        data: post_list,
        message: "User posts retrieved"
    }

fn create_post(title: str, content: str, author_id: i32) -> {}:
    """POST /posts - Create new post."""
    # Check user exists
    let user_exists = any(users, fn(u) => u.id == author_id)
    
    if not user_exists:
        return {
            status: 400,
            error: "Author not found"
        }
    
    let new_id = 1 + reduce(posts, 0, fn(acc, p) => max(acc, p.id))
    let post = Post.new(new_id, title, content, author_id)
    posts.push(post)
    
    return {
        status: 201,
        data: post.to_json(),
        message: "Post created successfully"
    }

fn like_post(post_id: i32) -> {}:
    """POST /posts/:id/like - Like a post."""
    for post in posts:
        if post.id == post_id:
            let likes = post.like()
            return {
                status: 200,
                data: {post_id: post_id, likes: likes},
                message: "Post liked"
            }
    
    return {
        status: 404,
        error: "Post not found"
    }

fn get_stats() -> {}:
    """GET /stats - Get API statistics."""
    let total_likes = reduce(posts, 0, fn(acc, p) => acc + p.likes)
    let avg_likes = total_likes / len(posts)
    
    return {
        status: 200,
        data: {
            total_users: len(users),
            total_posts: len(posts),
            total_likes: total_likes,
            avg_likes: avg_likes
        }
    }

# Request router
fn route(method: str, path: str, body: {} = {}) -> {}:
    """Route HTTP requests to handlers."""
    
    if path == "/users":
        if method == "GET":
            return get_users()
        else if method == "POST":
            return create_user(body.username, body.email)
    
    else if path == "/posts":
        if method == "GET":
            return get_posts()
        else if method == "POST":
            return create_post(body.title, body.content, body.author_id)
    
    else if path == "/stats":
        return get_stats()
    
    else if path.starts_with("/users/") and path.ends_with("/posts"):
        let user_id = parseInt(extract_path_param(path, 1))
        return get_user_posts(user_id)
    
    else if path.starts_with("/users/"):
        let user_id = parseInt(extract_path_param(path, 1))
        if method == "GET":
            return get_user(user_id)
    
    else if path.starts_with("/posts/") and path.ends_with("/like"):
        let post_id = parseInt(extract_path_param(path, 1))
        return like_post(post_id)
    
    return {
        status: 404,
        error: "Endpoint not found"
    }

# Utility function
fn extract_path_param(path: str, position: i32) -> str:
    """Extract path parameter."""
    let parts = split(path, "/")
    return parts[position]

fn format_response(response: {}) -> str:
    """Format response as JSON."""
    return json.stringify(response, pretty: true)

# ========================================
fn main():
    print("=" * 60)
    print("RESTful API Server - Apex Edition")
    print("=" * 60)
    
    # Simulated API requests
    let requests = [
        {method: "GET", path: "/users"},
        {method: "GET", path: "/users/1"},
        {method: "GET", path: "/users/1/posts"},
        {method: "POST", path: "/users", body: {username: "dave", email: "dave@example.com"}},
        {method: "GET", path: "/posts"},
        {method: "POST", path: "/posts", body: {title: "New Post", content: "Content", author_id: 1}},
        {method: "POST", path: "/posts/1/like"},
        {method: "POST", path: "/posts/2/like"},
        {method: "GET", path: "/stats"}
    ]
    
    for i, request in enumerate(requests):
        print("\n" + "-" * 60)
        print("Request ${i + 1}: ${request.method} ${request.path}")
        print("-" * 60)
        
        let response = route(request.method, request.path, request.body)
        
        if response.status >= 200 and response.status < 300:
            print("✅ Status: ${response.status}")
            print("Message: ${response.message}")
            if response.data:
                print("Data: ${format_response(response.data)}")
        else:
            print("❌ Status: ${response.status}")
            print("Error: ${response.error}")
    
    print("\n" + "=" * 60)
    print("✅ API Demo Complete!")
    print("=" * 60)