'use client';

import { useEffect, useState, ReactNode, JSX } from 'react';
import { useRouter } from 'next/navigation';
import { jwtDecode } from 'jwt-decode';
import { useAuth } from '@/core/auth/useAuth';
import Loading from './Loading';

interface AuthGuardProps {
    children: ReactNode;
}

const isTokenValid = (): boolean => {
    const token = localStorage.getItem('token');

    if (!token) {
        return false;
    }

    try {
        const decodedToken: any = jwtDecode(token); // Decode the token
        const currentTime = Date.now() / 1000; // Get current time in seconds

        // Check if the token has expired
        return decodedToken.exp && decodedToken.exp > currentTime;
    } catch (error) {
        console.error('Error decoding token:', error);
        return false;
    }
};

const AuthGuard = ({ children }: AuthGuardProps): JSX.Element | null => {
    const auth = useAuth();
    const router = useRouter();
    const [isValid, setIsValid] = useState<boolean>(false);

    useEffect(() => {
        console.log(auth, isTokenValid());
        if (!auth.user && !isTokenValid()) {
            // Redirect to login or unauthorized page if token is invalid or user is not authenticated
            router.push('/login');
        } else {
            setIsValid(true);
        }
    }, [auth.user, router]);

    // Render children only if the token is valid and user is authenticated
    if (isValid && !auth.loading) {
        return <>{children}</>;
    }

    // Optionally, you can show a loading state until the check is complete
    return <Loading />;
};

export default AuthGuard;
