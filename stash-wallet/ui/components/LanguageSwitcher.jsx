import React, { useState, useEffect } from 'react';
import { languages } from '../utils/languages';

const LanguageContext = React.createContext();

export const LanguageProvider = ({ children }) => {
  const [language, setLanguage] = useState('en');

  useEffect(() => {
    const savedLang = localStorage.getItem('language') || 'en';
    setLanguage(savedLang);
  }, []);

  const t = (key) => {
    const langData = require(`../locales/${language}.json`);
    return langData[key] || key;
  };

  const changeLanguage = (lang) => {
    setLanguage(lang);
    localStorage.setItem('language', lang);
    window.location.reload();
  };

  return (
    <LanguageContext.Provider value={{ language, t, changeLanguage }}>
      {children}
    </LanguageContext.Provider>
  );
};

export const useTranslation = () => React.useContext(LanguageContext);