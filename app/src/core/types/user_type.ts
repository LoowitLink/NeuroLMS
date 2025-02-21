export default interface User {
    id: any;
    name: string;
    pref_name: string;
    email: string;
    avatar: string;
    permissions: [];
    role: string;
    enrolled_courses: [];
    status: string;
    last_login: number;
    created_at: number;
    updated_at: number;
}