using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Food : MonoBehaviour
{
    public float food = 0;

    FoodParams config;

    private void Start() {
        var body = GetComponent<Rigidbody2D>();
        body.AddForce(new Vector2(config.velocity.sample(), config.velocity.sample()));
    }

    void Awake()
    {
        config = GameObject.Find("Settings").GetComponent<Settings>().foodParams;
        food = config.value.sample();
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
