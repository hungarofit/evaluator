<?php

namespace Hungarofit\Evaluator;


class Text
{
    public static function camelcase($text)
    {
        $text = ucfirst($text);
        return preg_replace_callback('/-([a-z0-9]+)/', function($m) {
            return ucfirst($m[1]);
        }, $text);
    }
    public static function kebabcase($text)
    {
        if(!$text) {
            return $text;
        }
        $text[0] = strtolower($text[0]);
        return preg_replace_callback('/([A-Z]|[0-9][a-z0-9]+)/', function($m) {
            return '-' . strtolower($m[1]);
        }, $text);
    }
}