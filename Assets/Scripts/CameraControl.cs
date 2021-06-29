using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CameraControl : MonoBehaviour
{
    Camera cam;
    Vector3 clickPos;
    Vector3 initCamPos;
    private Vector3 lastPosition;
    public float targetOrtho;
    public float minOrtho = 1.0f;
    float maxOrtho = 20.0f;
    public float zoomSpeed = 1;

    // Start is called before the first frame update
    void Start()
    {
        cam = GetComponent<Camera>();        
        targetOrtho = Camera.main.orthographicSize;
        maxOrtho = 1000;
    }

 
    void Update()
    {
        if (Input.GetMouseButtonDown(2))
        {
            lastPosition = cam.ScreenToWorldPoint(Input.mousePosition);
        }
 
        if (Input.GetMouseButton(2))
        {
            var delta = cam.ScreenToWorldPoint(Input.mousePosition) - lastPosition;
            transform.Translate(-delta.x, -delta.y, 0);
            lastPosition = cam.ScreenToWorldPoint(Input.mousePosition);
        }

        float scroll = Input.GetAxis("Mouse ScrollWheel");
        if (scroll != 0.0f) {
            targetOrtho -= (scroll * zoomSpeed) * (targetOrtho / maxOrtho);
            targetOrtho = Mathf.Clamp(targetOrtho, minOrtho, maxOrtho);
        }
        Camera.main.orthographicSize = targetOrtho;
    }
}
