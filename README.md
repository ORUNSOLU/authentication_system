Routes/
    register (check if user not exists & store in the user db if true)
    login (generate session_token, store session_token in db)
    delete_user
    update_user (must be logged-in, then change user_name)

Types/
    user
    session

Databases/
    employees (employee_id, name, password, security_question)
    user (user_id, email, name, password, role)
    session_table (user_id, session_token)