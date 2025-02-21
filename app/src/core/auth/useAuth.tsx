'use client';
import { useContext } from 'react';
import { AuthContext } from '@/core/auth/AuthContext';

export const useAuth = () => useContext(AuthContext);
