import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { AuthorizationsPageComponent } from './authorizations-page.component';

describe('AuthorizationsPageComponent', () => {
  let component: AuthorizationsPageComponent;
  let fixture: ComponentFixture<AuthorizationsPageComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ AuthorizationsPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(AuthorizationsPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should be created', () => {
    expect(component).toBeTruthy();
  });
});
