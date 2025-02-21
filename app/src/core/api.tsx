export const server = process.env.NEXT_PUBLIC_SCHOLARA_API;
export default {
    auth: {
        login: () => `${server}/api/auth/login`,
        register: () => `${server}/api/auth/register`,
    },
    user: {
        account: {
            data: () => `${server}/api/user/account`,
        },
    },
};
