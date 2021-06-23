using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Food : MonoBehaviour
{
    public float _food;

    public float food {
        get => _food;
        set
        {
            _food = value;
            var color = sprite.color;
            color.a = (-1f / (_food / 20f + 1f)) + 1f;
            sprite.color = color;
        }
    }

    FoodParams config;
    SpriteRenderer sprite;

    void Awake()
    {
        sprite = GetComponent<SpriteRenderer>();
        config = GameObject.Find("Settings").GetComponent<Settings>().foodParams;
        food = config.value.sample();
    }

    private void Start() {
        var body = GetComponent<Rigidbody2D>();
        body.AddForce(new Vector2(config.velocity.sample(), config.velocity.sample()));
    }

    void OnCollisionEnter2D(Collision2D col)
    {
        if (gameObject.activeSelf == false) {
            return;
        }

        var foodObj = col.gameObject.GetComponent<Food>();
        if (foodObj == null) {
            return;
        }

        col.gameObject.SetActive(false);
        food += foodObj.food;
        GameObject.Destroy(col.gameObject);
    }
}
