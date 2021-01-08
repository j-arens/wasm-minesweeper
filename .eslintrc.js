module.exports = {
  extends: [
    'plugin:@typescript-eslint/recommended'
  ],

  overrides: [
    {
      files: [
        'webpack.config.js',
      ],
      rules: {
        '@typescript-eslint/no-var-requires': 0,
      },
    },
  ],

  parser: '@typescript-eslint/parser',

  rules: {
    'prefer-const': 0,
  }
};
