'use client';
import { createContext, useEffect, useState, ReactNode } from 'react';
import { useRouter } from 'next/navigation';

// ** Axios
import axios from 'axios';

import api from '@/core/api';
import Loading from './Loading';

// ** Types
interface User {
    _id?: any;
    name?: string;
    email?: string;
    role?: string;
    school?: string;
    orgs?: [];
    active?: boolean;
    exp: number;
    // Add more fields if necessary
}

interface AuthContextType {
    user: User | null;
    token: string | null;
    loading: boolean;
    setLoading: (loading: boolean) => void;
    setUser: (user: User | null) => void;
    handleLogout: () => void;
}

const defaultProvider: AuthContextType = {
    user: null,
    token: null,
    loading: true,
    setUser: () => null,
    setLoading: () => undefined,
    handleLogout: () => undefined,
};

const AuthContext = createContext<AuthContextType>(defaultProvider);

interface AuthProviderProps {
    children: ReactNode;
}

const AuthProvider = ({ children }: AuthProviderProps) => {
    // ** States
    const [userData, setUser] = useState<User | null>(defaultProvider.user);
    const [userToken, setUserToken] = useState<string | null>(defaultProvider.token);
    const [loading, setLoading] = useState<boolean>(defaultProvider.loading);

    // ** Hooks
    const router = useRouter();
    useEffect(() => {
        const userGlobalToken = window.localStorage.getItem('token');
        setUserToken(userGlobalToken);

        const initAuth = async () => {
            const storedToken = window.localStorage.getItem('token');

            if (storedToken) {
                setLoading(true);
                await axios
                    .get(api.user.account.data(), {
                        headers: {
                            Authorization: storedToken,
                            'Cache-Control': 'max-age=5400',
                        },
                    })
                    .then(async (response) => {
                        setUser(response.data.user);
                        setLoading(false);
                    })
                    .catch(() => {
                        localStorage.removeItem('user');
                        localStorage.removeItem('refreshToken');
                        localStorage.removeItem('accessToken');
                        setUser(null);
                        setLoading(false);
                        router.push('/s'); // Redirect to login page
                    });
            } else {
                setLoading(false);
                router.push('/login'); // Redirect to login page if no token
            }
        };
        initAuth();
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    const handleLogout = () => {
        window.localStorage.removeItem('user');
        window.localStorage.removeItem('token');
        window.localStorage.clear();
        setUser(null);
        router.push('/login');
    };

    const values: AuthContextType = {
        user: userData,
        token: userToken,
        loading,
        setLoading,
        setUser,
        handleLogout,
    };

    return <AuthContext.Provider value={values}>{loading ? <Loading /> : children}</AuthContext.Provider>;
};

export { AuthContext, AuthProvider };
