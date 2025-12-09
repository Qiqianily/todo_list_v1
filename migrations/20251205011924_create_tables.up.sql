-- Add up migration script here
-- 创建用户表
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100),
    avatar_url VARCHAR(255),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_login_at TIMESTAMP WITH TIME ZONE
);

-- 创建更新时间触发器
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- 创建索引
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at);

-- 创建待办事项表
CREATE TABLE todo_list (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    summary TEXT[],
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'in_progress', 'completed', 'cancelled')),
    priority VARCHAR(10) DEFAULT 'medium' CHECK (priority IN ('low', 'medium', 'high', 'urgent')),
    due_date TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    is_important BOOLEAN DEFAULT false,
    is_urgent BOOLEAN DEFAULT false,
    tags TEXT[], -- 使用数组存储标签
    estimated_time INTEGER, -- 预估分钟数
    actual_time INTEGER, -- 实际花费分钟数
    parent_id INTEGER, -- 用于子任务
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    -- 外键约束
    CONSTRAINT fk_todo_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_todo_parent FOREIGN KEY (parent_id) REFERENCES todo_list(id) ON DELETE CASCADE
);

-- 创建更新时间触发器
CREATE TRIGGER update_todo_updated_at
    BEFORE UPDATE ON todo_list
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- 创建索引
CREATE INDEX idx_todo_user_id ON todo_list(user_id);
CREATE INDEX idx_todo_status ON todo_list(status);
CREATE INDEX idx_todo_priority ON todo_list(priority);
CREATE INDEX idx_todo_due_date ON todo_list(due_date);
CREATE INDEX idx_todo_completed_at ON todo_list(completed_at);
CREATE INDEX idx_todo_created_at ON todo_list(created_at);
CREATE INDEX idx_todo_parent_id ON todo_list(parent_id);
CREATE INDEX idx_todo_user_status ON todo_list(user_id, status);
CREATE INDEX idx_todo_user_priority ON todo_list(user_id, priority);

-- 用于标签搜索的GIN索引
CREATE INDEX idx_todo_tags ON todo_list USING GIN(tags);
